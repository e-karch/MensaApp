
import '../../repository/data_classes/filter/FilterPreferences.dart';
import '../../repository/data_classes/mealplan/Canteen.dart';
import '../../repository/data_classes/settings/ColorScheme.dart';
import '../../repository/data_classes/settings/MealPlanFormat.dart';
import '../../repository/data_classes/settings/PriceCategory.dart';

/// This is an interface for accessing the preferences.
abstract class IPreferenceAccess {
  /// The device identifier is returned.
  /// @return The device identifier.
  Future<String> getClientIdentifier();

  /// The device identifier is set.
  /// @param identifier The new device identifier.
  /// @return The result of the update.
  Future<void> setClientIdentifier(String identifier);

  /// The saved FilterPreferences is returned.
  /// @return The saved FilterPreferences.
  Future<FilterPreferences> getFilterPreferences();

  /// The committed FilterPreferences is set.
  /// @param filter The new FilterPreferences.
  /// @return The result of the update.
  Future<void> setFilterPreferences(FilterPreferences filter);

  /// The FilterPreferences are reset to the default values.
  /// @return The result of the update.
  Future<void> resetFilterPreferences();

  /// The saved Canteen is returned.
  /// @return The saved Canteen.
  Future<Canteen> getCanteen();

  /// The committed Canteen is set.
  /// @param canteen The new Canteen.
  /// @return The result of the update.
  Future<void> setCanteen(Canteen canteen);

  /// The saved ColorScheme is returned.
  /// @return The saved ColorScheme.
  Future<ColorScheme> getColorScheme();

  /// The committed ColorScheme is set.
  /// @param scheme The new ColorScheme.
  /// @return The result of the update.
  Future<void> setColorScheme(ColorScheme scheme);

  /// The saved PriceCategory is returned.
  /// @return The saved PriceCategory.
  Future<PriceCategory> getPriceCategory();

  /// The committed PriceCategory is set.
  /// @param category The new PriceCategory.
  /// @return The result of the update.
  Future<void> setPriceCategory(PriceCategory category);

  /// The saved MealPlanFormat is returned.
  /// @return The saved MealPlanFormat.
  Future<MealPlanFormat> getMealPlanFormat();

  /// The committed MealPlanFormat is set.
  /// @param format The new MealPlanFormat.
  /// @return The result of the update.
  Future<void> setMealPlanFormat(MealPlanFormat format);
}