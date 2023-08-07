import 'package:app/model/database/model/database_model.dart';

import '../../../view_model/repository/data_classes/meal/FoodType.dart';

/// This class represents a meal in the database.
class DBMeal implements DatabaseModel {
  final String _mealID;
  final String _name;
  final FoodType _foodType;
  final int _individualRating;
  final int _numberOfRatings;
  final double _averageRating;

  /// The name of the table in the database.
  static const String tableName = 'meal';

  /// The name of the column for the meal id.
  static const String columnMealID = 'mealID';

  /// The name of the column for the meal plan id.
  static const String columnMealPlanID = 'mealPlanID';

  /// The name of the column for the name.
  static const String columnName = 'name';

  /// The name of the column for the food type.
  static const String columnFoodType = 'foodType';

  /// The name of the column for the individual rating.
  static const String columnIndividualRating = 'individualRating';

  /// The name of the column for the number of ratings.
  static const String columnNumberOfRatings = 'numberOfRatings';

  /// The name of the column for the average rating.
  static const String columnAverageRating = 'averageRating';

  /// Creates a new instance of a meal.
  /// @param _mealID The id of the meal.
  /// @param _mealPlanID The id of the meal plan.
  /// @param _name The name of the meal.
  /// @param _foodType The food type of the meal.
  /// @param _priceStudent The price for students.
  /// @param _priceEmployee The price for employees.
  /// @param _pricePupil The price for pupils.
  /// @param _priceGuest The price for guests.
  /// @param _individualRating The individual rating.
  /// @param _numberOfRatings The number of ratings.
  /// @param _averageRating The average rating.
  /// @param _lastServed The last served date.
  /// @param _nextServed The next served date.
  /// @param _relativeFrequency The relative frequency.
  /// @return A new instance of a meal.
  DBMeal(this._mealID, this._name, this._foodType, this._individualRating,
      this._numberOfRatings, this._averageRating);

  @override
  Map<String, dynamic> toMap() {
    return {
      columnMealID: _mealID,
      columnName: _name,
      columnFoodType: _foodType.name,
      columnIndividualRating: _individualRating,
      columnNumberOfRatings: _numberOfRatings,
      columnAverageRating: _averageRating,
    };
  }

  /// Creates a new instance of a meal from a map.
  /// @param map The map to create a meal from.
  /// @return A new instance of a meal.
  static DBMeal fromMap(Map<String, dynamic> map) {
    return DBMeal(
        map[columnMealID],
        map[columnName],
        FoodType.values.byName(map[columnFoodType]),
        map[columnIndividualRating],
        map[columnNumberOfRatings],
        _checkDouble(map[columnAverageRating]) ?? 0);
  }

  /// The string to create a table for a meal.
  /// @return The string to create a table for a meal.
  static String initTable() {
    return '''
    CREATE TABLE $tableName (
      $columnMealID TEXT PRIMARY KEY,
      $columnName TEXT NOT NULL,
      $columnFoodType TEXT NOT NULL,
      $columnIndividualRating INTEGER,
      $columnNumberOfRatings INTEGER NOT NULL,
      $columnAverageRating DECIMAL(1,1)
    )
  ''';
  }

  /// This method is used to get the average rating of the meal.
  /// @return The average rating of the meal.
  double get averageRating => _averageRating;

  /// This method is used to get the number of ratings.
  /// @return The number of ratings.
  int get numberOfRatings => _numberOfRatings;

  /// This method is used to get the individual rating of the meal.
  /// @return The individual rating of the meal.
  int get individualRating => _individualRating;

  /// This method is used to get the food type of the meal.
  /// @return The food type of the meal.
  FoodType get foodType => _foodType;

  /// This method is used to get the name of the meal.
  /// @return The name of the meal.
  String get name => _name;

  /// This method is used to get the meal id.
  /// @return The meal id.
  String get mealID => _mealID;

  static double? _checkDouble(dynamic value) {
    if (value is double) return value;
    if (value is int) return value.toDouble();
    if (value is String) return double.tryParse(value);
    return null;
  }
}
