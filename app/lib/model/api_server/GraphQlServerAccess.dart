import 'package:app/model/api_server/requests/querys.graphql.dart';
import 'package:app/model/api_server/requests/schema.graphql.dart';
import 'package:app/view_model/repository/data_classes/filter/Frequency.dart';
import 'package:app/view_model/repository/data_classes/meal/Additive.dart';
import 'package:app/view_model/repository/data_classes/meal/Allergen.dart';
import 'package:app/view_model/repository/data_classes/meal/FoodType.dart';
import 'package:app/view_model/repository/data_classes/meal/ImageData.dart';

import 'package:app/view_model/repository/data_classes/meal/Meal.dart';
import 'package:app/view_model/repository/data_classes/meal/Price.dart';
import 'package:app/view_model/repository/data_classes/meal/Side.dart';

import 'package:app/view_model/repository/data_classes/mealplan/Canteen.dart';
import 'package:app/view_model/repository/data_classes/mealplan/Line.dart';

import 'package:app/view_model/repository/data_classes/mealplan/MealPlan.dart';

import 'package:app/view_model/repository/data_classes/settings/ReportCategory.dart';
import 'package:app/view_model/repository/error_handling/NoMealException.dart';

import 'package:app/view_model/repository/error_handling/Result.dart';
import 'package:flutter/cupertino.dart';
import 'package:graphql_flutter/graphql_flutter.dart';
import 'package:intl/intl.dart';

import '../../view_model/repository/interface/IServerAccess.dart';
import 'requests/mutations.graphql.dart';

class GraphQlServerAccess implements IServerAccess {
  final String _apiKey = const String.fromEnvironment('API_KEY');

  final GraphQLClient _client = GraphQLClient(
      link: HttpLink(const String.fromEnvironment('API_URL')),
      cache: GraphQLCache());
  final String _clientId;
  final _dateFormat = DateFormat(dateFormatPattern);

  GraphQlServerAccess._(this._clientId);

  factory GraphQlServerAccess(String clientId) {
    return GraphQlServerAccess._(clientId);
  }

  // ---------------------- mutations ----------------------

  @override
  Future<bool> deleteDownvote(ImageData image) async {
    // TODO auth
    final result = await _client.mutate$RemoveDownvote(
        Options$Mutation$RemoveDownvote(
            variables: Variables$Mutation$RemoveDownvote(imageId: image.id)));
    final parsedData = result.parsedData;
    return parsedData?.removeDownvote ?? false;
  }

  @override
  Future<bool> deleteUpvote(ImageData image) async {
    // TODO auth
    final result = await _client.mutate$RemoveUpvote(
        Options$Mutation$RemoveUpvote(
            variables: Variables$Mutation$RemoveUpvote(imageId: image.id)));

    return result.parsedData?.removeUpvote ?? false;
  }

  @override
  Future<bool> downvoteImage(ImageData image) async {
    // TODO auth
    final result = await _client.mutate$AddDownvote(
        Options$Mutation$AddDownvote(
            variables: Variables$Mutation$AddDownvote(imageId: image.id)));

    return result.parsedData?.addDownvote ?? false;
  }

  @override
  Future<bool> upvoteImage(ImageData image) async {
    // TODO auth
    final result = await _client.mutate$AddUpvote(Options$Mutation$AddUpvote(
        variables: Variables$Mutation$AddUpvote(imageId: image.id)));

    return result.parsedData?.addUpvote ?? false;
  }

  @override
  Future<bool> linkImage(String url, Meal meal) async {
    // TODO: auth
    final result = await _client.mutate$LinkImage(Options$Mutation$LinkImage(
        variables:
            Variables$Mutation$LinkImage(imageUrl: url, mealId: meal.id)));

    return result.parsedData?.addImage ?? false;
  }

  @override
  Future<bool> reportImage(ImageData image, ReportCategory reportReason) async {
    // TODO: auth
    final result = await _client.mutate$ReportImage(
        Options$Mutation$ReportImage(
            variables: Variables$Mutation$ReportImage(
                imageId: image.id,
                reason: _convertToReportReason(reportReason))));

    return result.parsedData?.reportImage ?? false;
  }

  @override
  Future<bool> updateMealRating(int rating, Meal meal) async {
    // TODO: auth
    final result = await _client.mutate$UpdateRating(
        Options$Mutation$UpdateRating(
            variables: Variables$Mutation$UpdateRating(
                mealId: meal.id, rating: rating)));

    return result.parsedData?.setRating ?? false;
  }

  // ---------------------- queries ----------------------
  static const daysToParse = 7;
  static const dateFormatPattern = "yyyy-MM-dd";

  @override
  Future<Result<List<Mealplan>>> updateAll() async {
    final today = DateTime.now();

    var completeList = <Mealplan>[];

    // TODO parallel?
    for (int offset = 0; offset < daysToParse; offset++) {

      final date = today.add(Duration(days: offset));
      final result = await _client.query$GetMealPlanForDay(
          Options$Query$GetMealPlanForDay(
              variables: Variables$Query$GetMealPlanForDay(
                  date: _dateFormat.format(date))));

      final exception = result.exception;
      if (exception != null) {
        return Failure(exception);
      }

      final mealPlan = _convertMealPlan(result.parsedData?.getCanteens ?? [], date);

      completeList.addAll(mealPlan);
    }
    return Success(completeList); // TODO when return error?
  }

  @override
  Future<Result<Meal>> getMealFromId(
      Meal meal, Line line, DateTime date) async {
    final result = await _client.query$GetMeal(Options$Query$GetMeal(
        variables: Variables$Query$GetMeal(
            date: _dateFormat.format(date), mealId: meal.id, lineId: line.id)));

    final mealData = result.parsedData?.getMeal;
    final exception = result.exception;
    if (exception != null) {
      return Failure(exception);
    }

    if (mealData == null) {
      return Failure(NoMealException(
          "Could not request meal from api: ${result.exception}"));
    }

    return Success(_convertMeal(mealData));
  }

  @override
  Future<Result<List<Mealplan>>> updateCanteen(
      Canteen canteen, DateTime date) async {
    final result = await _client.query$GetCanteenDate(
        Options$Query$GetCanteenDate(
            variables: Variables$Query$GetCanteenDate(
                canteenId: canteen.id, date: _dateFormat.format(date))));

    final exception = result.exception;
    if (exception != null) {
      return Failure(exception);
    }

    final mealPlan =
        _convertMealPlan([result.parsedData?.getCanteen].nonNulls.toList(), date);
    return Success(mealPlan);
  }
}

// --------------- utility helper methods ---------------

List<Mealplan> _convertMealPlan(
    List<Fragment$mealPlan> mealPlan, DateTime date) {
  return mealPlan
      .expand(
        (e) => e.lines
            .asMap()
            .map((idx, e) => MapEntry(
                  idx,
                  Mealplan(
                    date: date,
                    line: Line(
                        id: e.id,
                        name: e.name,
                        canteen: _convertCanteen(e.canteen),
                        position: idx),
                    // mensa closed when data available but no meals in list
                    isClosed: e.meals?.isEmpty ?? false,
                    meals: e.meals?.map((e) => _convertMeal(e)).toList() ?? [],
                  ),
                ))
            .values
            .toList(),
      )
      .toList();
}

Meal _convertMeal(Fragment$mealInfo meal) {
  return Meal(
    id: meal.id,
    name: meal.name,
    foodType: _convertMealType(meal.mealType),
    price: _convertPrice(meal.price),
    additives: meal.additives.map((e) => _convertAdditive(e)).nonNulls.toList(),
    allergens:
        meal.allergens.map((e) => _convertAllergen(e)).nonNulls.toList(),
    averageRating: meal.ratings.averageRating,
    individualRating: meal.ratings.personalRating,
    numberOfRatings: meal.ratings.ratingsCount,
    lastServed: _convertDate(meal.statistics.lastServed),
    nextServed: _convertDate(meal.statistics.nextServed),
    relativeFrequency: _specifyFrequency(meal.statistics.relativeFrequency),
    images: meal.images.map((e) => _convertImage(e)).toList(),
    sides: meal.sides.map((e) => _convertSide(e)).toList(),
  );
}

Frequency _specifyFrequency(double frequency) {
  throw UnsupportedError("message");
}

DateTime? _convertDate(String? date) {
  final format = DateFormat(GraphQlServerAccess.dateFormatPattern);
  try {
    return format.parse(date ?? "");
  } catch (e) {
    return null;
  }
}

Side _convertSide(Fragment$mealInfo$sides e) {
  return Side(
      id: e.id,
      name: e.name,
      foodType: _convertMealType(e.mealType),
      price: _convertPrice(e.price),
      allergens: e.allergens.map((e) => _convertAllergen(e)).nonNulls.toList(),
      additives: e.additives.map((e) => _convertAdditive(e)).nonNulls.toList());
}

ImageData _convertImage(Fragment$mealInfo$images e) {
  return ImageData(
      id: e.id,
      url: e.url,
      imageRank: e.rank,
      positiveRating: e.upvotes,
      negativeRating: e.downvotes,
      individualRating: e.personalUpvote
          ? 1
          : e.personalDownvote
              ? -1
              : 0);
}

Allergen? _convertAllergen(Enum$Allergen e) {
  return switch (e) {
    Enum$Allergen.CA => Allergen.ca,
    Enum$Allergen.DI => Allergen.di,
    Enum$Allergen.EI => Allergen.ei,
    Enum$Allergen.ER => Allergen.er,
    Enum$Allergen.FI => Allergen.fi,
    Enum$Allergen.GE => Allergen.ge,
    Enum$Allergen.HF => Allergen.hf,
    Enum$Allergen.HA => Allergen.ha,
    Enum$Allergen.KA => Allergen.ka,
    Enum$Allergen.KR => Allergen.kr,
    Enum$Allergen.LU => Allergen.lu,
    Enum$Allergen.MA => Allergen.ma,
    Enum$Allergen.ML => Allergen.ml,
    Enum$Allergen.PA => Allergen.pa,
    Enum$Allergen.PE => Allergen.pe,
    Enum$Allergen.PI => Allergen.pi,
    Enum$Allergen.QU => Allergen.qu,
    Enum$Allergen.RO => Allergen.ro,
    Enum$Allergen.SA => Allergen.sa,
    Enum$Allergen.SE => Allergen.se,
    Enum$Allergen.SF => Allergen.sf,
    Enum$Allergen.SN => Allergen.sn,
    Enum$Allergen.SO => Allergen.so,
    Enum$Allergen.WA => Allergen.wa,
    Enum$Allergen.WE => Allergen.we,
    Enum$Allergen.WT => Allergen.wt,
    Enum$Allergen.LA => Allergen.la,
    Enum$Allergen.GL => Allergen.gl,
    Enum$Allergen.$unknown => null,
  };
}

Additive? _convertAdditive(Enum$Additive e) {
  return switch (e) {
    Enum$Additive.COLORANT => Additive.colorant,
    Enum$Additive.PRESERVING_AGENTS => Additive.preservingAgents,
    Enum$Additive.ANTIOXIDANT_AGENTS => Additive.antioxidantAgents,
    Enum$Additive.FLAVOUR_ENHANCER => Additive.flavourEnhancer,
    Enum$Additive.PHOSPHATE => Additive.phosphate,
    Enum$Additive.SURFACE_WAXED => Additive.surfaceWaxed,
    Enum$Additive.SULPHUR => Additive.sulphur,
    Enum$Additive.ARTIFICIALLY_BLACKENED_OLIVES =>
      Additive.artificiallyBlackenedOlives,
    Enum$Additive.SWEETENER => Additive.sweetener,
    Enum$Additive.LAXATIVE_IF_OVERUSED => Additive.laxativeIfOverused,
    Enum$Additive.PHENYLALANINE => Additive.phenylalanine,
    Enum$Additive.ALCOHOL => Additive.alcohol,
    Enum$Additive.PRESSED_MEET => Additive.pressedMeat,
    Enum$Additive.GLAZING_WITH_CACAO => Additive.glazingWithCacao,
    Enum$Additive.PRESSED_FISH => Additive.pressedFish,
    Enum$Additive.$unknown => null,
  };
}

FoodType _convertMealType(Enum$MealType mealType) {
  switch (mealType) {
    case Enum$MealType.BEEF:
      return FoodType.beef;
    case Enum$MealType.BEEF_AW:
      return FoodType.beefAw;
    case Enum$MealType.FISH:
      return FoodType.fish;
    case Enum$MealType.PORK:
      return FoodType.pork;
    case Enum$MealType.PORK_AW:
      return FoodType.porkAw;
    case Enum$MealType.VEGAN:
      return FoodType.vegan;
    case Enum$MealType.VEGETARIAN:
      return FoodType.vegetarian;
    case Enum$MealType.UNKNOWN:
      return FoodType.unknown;
    case Enum$MealType.$unknown:
      return FoodType.unknown;
  }
}

Price _convertPrice(Fragment$price price) {
  return Price(
      student: price.student,
      employee: price.employee,
      pupil: price.pupil,
      guest: price.guest);
}

Canteen _convertCanteen(Fragment$mealPlan$lines$canteen canteen) {
  return Canteen(id: canteen.id, name: canteen.name);
}

Enum$ReportReason _convertToReportReason(ReportCategory reportReason) {
  switch (reportReason) {
    case ReportCategory.offensive:
      return Enum$ReportReason.OFFENSIVE;
    case ReportCategory.advert:
      return Enum$ReportReason.ADVERT;
    case ReportCategory.noMeal:
      return Enum$ReportReason.NO_MEAL;
    case ReportCategory.violatesRights:
      return Enum$ReportReason.VIOLATES_RIGHTS;
    case ReportCategory.wrongMeal:
      return Enum$ReportReason.WRONG_MEAL;
    case ReportCategory.other:
      return Enum$ReportReason.OTHER;
  }
}
