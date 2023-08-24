import 'dart:js_util';

import 'package:app/model/api_server/GraphQlServerAccess.dart';
import 'package:app/model/database/SQLiteDatabaseAccess.dart';
import 'package:app/model/local_storage/SharedPreferenceAccess.dart';
import 'package:app/view_model/logic/meal/CombinedMealPlanAccess.dart';
import 'package:app/view_model/repository/data_classes/meal/Additive.dart';
import 'package:app/view_model/repository/data_classes/meal/Allergen.dart';
import 'package:app/view_model/repository/data_classes/meal/ImageData.dart';
import 'package:app/view_model/repository/data_classes/meal/Meal.dart';
import 'package:app/view_model/repository/data_classes/meal/Side.dart';
import 'package:app/view_model/repository/data_classes/mealplan/MealPlan.dart';
import 'package:app/view_model/repository/error_handling/Result.dart';
import 'package:flutter_test/flutter_test.dart';
import 'package:shared_preferences/shared_preferences.dart';

import '../model/api_server/config.dart';

Future<void> main() async {
  final Map<String, Object> values = <String, Object>{'counter': 1};
  SharedPreferences.setMockInitialValues(values);

  SQLiteDatabaseAccess database = SQLiteDatabaseAccess();
  GraphQlServerAccess api = GraphQlServerAccess(
      testServer, testApiKey, "1f16dcca-963e-4ceb-a8ca-843a7c9277a5");
  SharedPreferenceAccess localStorage =
      SharedPreferenceAccess(await SharedPreferences.getInstance());

  CombinedMealPlanAccess access =
      CombinedMealPlanAccess(localStorage, api, database);
  List<MealPlan> mealplan = [];

  setUp(() async {
    mealplan = switch (await api.updateCanteen(
        await access.getCanteen(), await access.getDate())) {
      Success(value: final value) => value,
      Failure(exception: _) => []
    };
  });

  test("get meal plan", () async {
    final List<MealPlan> result = switch (await access.getMealPlan()) {
      Success(value: final value) => value,
      Failure(exception: _) => []
    };

    _listEquals(result, mealplan);
  });

  test("get meal", () async {
    final meal = mealplan.first.meals.first;
    final result = switch (await access.getMeal(meal)) {
      Success(value: final value) => value,
      Failure(exception: _) => null
    };

    expect(result is Meal, isTrue);
    _compareFullMeal(result!, meal);
  });

  test("get available canteens", () async {
    final result = await access.getAvailableCanteens();

    expect(result, hasLength(3));
  });

  test("refresh mealplan", () async {
    final result = await access.refreshMealplan();

    expect(result, not(null));
  });
}

void _compareFullMeal(Meal actual, Meal expected) {
  expect(actual.id, expected.id);
  expect(actual.name, expected.name);
  expect(actual.foodType, expected.foodType);
  expect(actual.price, expected.price);
  _listEquals<Allergen>(actual.allergens ?? [], expected.allergens ?? []);
  _listEquals<Additive>(actual.additives ?? [], expected.additives ?? []);
  _listEquals<Side>(actual.sides ?? [], expected.sides ?? []);
  expect(actual.individualRating, expected.individualRating);
  expect(actual.numberOfRatings, expected.numberOfRatings);
  expect(actual.averageRating, expected.averageRating);
  expect(actual.lastServed, expected.lastServed);
  expect(actual.nextServed, expected.nextServed);
  expect(actual.relativeFrequency, expected.relativeFrequency);
  _listEquals<ImageData>(actual.images ?? [], expected.images ?? []);
  expect(actual.numberOfOccurance, expected.numberOfOccurance);
  expect(actual.isFavorite, expected.isFavorite);
}

void _listEquals<T>(List<T> actual, List<T> expected) {
  expect(actual.length, expected.length);

  for (final element in expected) {
    expect(actual.contains(element), isTrue);
  }
}
