import 'package:app/view/core/buttons/MensaTapable.dart';
import 'package:app/view/core/selection_components/MensaDropdown.dart';
import 'package:app/view/core/selection_components/MensaDropdownEntry.dart';
import 'package:app/view/filter/MensaSortSelectEntry.dart';
import 'package:flutter/material.dart';

class MensaSortSelect<T> extends StatelessWidget {
  final List<MensaSortSelectEntry<T>> _entries;
  final T _selectedEntry;
  final SortDirection _sortDirection;
  final Function(T) _onEntrySelected;
  final Function(SortDirection) _onSortDirectionSelected;

  const MensaSortSelect(
      {super.key,
      required List<MensaSortSelectEntry<T>> entries,
      required T selectedEntry,
      required SortDirection sortDirection,
      required Function(T) onEntrySelected,
      required Function(SortDirection) onSortDirectionSelected})
      : _entries = entries,
        _selectedEntry = selectedEntry,
        _onEntrySelected = onEntrySelected,
        _sortDirection = sortDirection,
        _onSortDirectionSelected = onSortDirectionSelected;

  @override
  Widget build(BuildContext context) {
    return Row(
      children: [
        MensaDropdown(
            onChanged: (v) => _onEntrySelected(v),
            value: _selectedEntry,
            items: _entries
                .map((e) => MensaDropdownEntry(
                      value: e.value,
                      label: e.label,
                    ))
                .toList()),
        MensaTapable(
          child: const Padding(
            padding: EdgeInsets.all(8),
          ),
          onTap: () => _onSortDirectionSelected(
              _sortDirection == SortDirection.ascending
                  ? SortDirection.descending
                  : SortDirection.ascending),
        )
      ],
    );
  }
}

enum SortDirection { ascending, descending }
