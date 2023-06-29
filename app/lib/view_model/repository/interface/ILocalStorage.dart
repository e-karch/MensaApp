
abstract class ILocalStorage {
  /// The device identifier is returned.
  Future<String> getClientIdentifier();

  /// The device identifier is set.
  Future<void> setClientIdentifier(String identifier);

  /// The saved FilterPreferences is returned.
  Future<FilterPreferences> getFilterPreferences();

  /// The committed FilterPreferences is set.
  Future<void> setFilterPreferences(FilterPreference filter);

  /// The saved Canteen is returned.
  Future<Canteen> getCanteen();

  /// The committed Canteen is set.
  Future<void> setCanteen(Canteen canteen);

  /// The saved ColorScheme is returned.
  Future<ColorScheme> getColorScheme();

  /// The committed ColorScheme is set.
  Future<void> setColorScheme(ColorScheme scheme);

  /// The saved PriceCategory is returned.
  Future<PriceCategory> getPriceCategory();

  /// The committed PriceCategory is set.
  Future<void> setPriceCategory(PriceCategory category);

  /// The saved MealPlanFormat is returned.
  Future<MealPlanFormat> getMealPlanFormat();

  /// The committed MealPlanFormat is set.
  Future<void> setMealPlanFormat(MealPlanFormat format);
}