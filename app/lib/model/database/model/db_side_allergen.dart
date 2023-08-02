import 'package:app/model/database/model/database_model.dart';
import 'package:app/model/database/model/db_side.dart';

import '../../../view_model/repository/data_classes/meal/Allergen.dart';

/// This class represents an allergen of a side in the database.
class DBSideAllergen implements DatabaseModel {
  final String _sideID;
  final Allergen _allergen;

  /// The name of the table in the database.
  static const String tableName = 'sideAllergen';

  /// The name of the column for the side id.
  static const String columnSideID = 'sideID';

  /// The name of the column for the allergen.
  static const String columnAllergen = 'allergen';

  /// Creates a new instance of a side allergen.
  /// @param _sideID The id of the side.
  /// @param _allergen The allergen of the side.
  /// @returns A new instance of a side allergen.
  DBSideAllergen(this._sideID, this._allergen);

  @override
  Map<String, dynamic> toMap() {
    return {columnSideID: _sideID, columnAllergen: _allergen.name};
  }

  /// Creates a new instance of a side allergen from a map.
  /// @param map The map to create the instance from.
  /// @returns A new instance of a side allergen.
  static DBSideAllergen fromMap(Map<String, dynamic> map) {
    return DBSideAllergen(map[columnSideID], Allergen.values.byName(map[columnAllergen]));
  }

  /// The string to create a table for an allergen of a side.
  /// @returns The string to create a table for an allergen of a side.
  static String initTable() {
    return '''
    CREATE TABLE $tableName (
      $columnSideID TEXT,
      $columnAllergen TEXT,
      FOREIGN KEY($columnSideID) REFERENCES ${DBSide.tableName}(${DBSide.columnSideID}),
      PRIMARY KEY($columnSideID, $columnAllergen)
    )
  ''';
  }

  /// This method returns the allergen of the side.
  /// @returns The allergen of the side.
  Allergen get allergen => _allergen;

  /// This method returns the id of the side.
  /// @returns The id of the side.
  String get sideID => _sideID;
}
