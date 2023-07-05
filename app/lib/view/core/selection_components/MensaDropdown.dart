import 'package:app/view/core/selection_components/MensaDropdownEntry.dart';
import 'package:flutter/material.dart';

/// A dropdown that is used in the Mensa app.
class MensaDropdown<T> extends StatelessWidget {
  final void Function(T?)? _onChanged;
  final T _value;
  final List<MensaDropdownEntry<T>> _items;

  /// Creates a new MensaDropdown.
  /// @param key The key to identify this widget.
  /// @param onChanged The function that is called when the value changes.
  /// @param value The value that is currently selected.
  /// @param items The items that can be selected.
  /// @returns A new MensaDropdown.
  const MensaDropdown(
      {super.key,
      required Function(T?)? onChanged,
      required T value,
      required List<MensaDropdownEntry<T>> items})
      : _onChanged = onChanged,
        _value = value,
        _items = items;

  /// Builds the widget.
  /// @param context The context in which the widget is built.
  /// @returns The widget.
  @override
  Widget build(BuildContext context) {
    return Container( // Container is used to give the dropdown a background color.
        decoration: BoxDecoration(
          borderRadius: BorderRadius.circular(4.0),
          color: Theme.of(context).colorScheme.surface,
        ),
        child: Padding(
            padding: const EdgeInsets.symmetric(horizontal: 16.0),
            child: DropdownButtonHideUnderline( // DropdownButtonHideUnderline is used to hide the underline of the dropdown.
                child: DropdownButton<T>(
              elevation: 0,
              borderRadius: BorderRadius.circular(4.0),
              value: _value,
              onChanged: _onChanged,
              items: _items.map((e) => e.build(context)).toList(),
            ))));
  }
}
