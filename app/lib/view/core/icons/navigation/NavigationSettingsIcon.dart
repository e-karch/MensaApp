import 'package:flutter/material.dart';
import 'package:flutter_svg/flutter_svg.dart';

/// This widget is used to display the icon for the settings
class NavigationSettingsIcon extends StatelessWidget {
  final double? _size;
  final Color? _color;

  /// This widget is used to display the icon for the settings
  /// @param key is the key of the widget
  /// @param size is the size of the icon
  /// @param color is the color of the icon
  /// @returns a [NavigationSettingsIcon] widget
  const NavigationSettingsIcon({super.key, double size = 24, Color? color})
      : _size = size,
        _color = color;

  @override
  Widget build(BuildContext context) {
    return SvgPicture.asset(
      'assets/icons/navigation/nav_settings.svg',
      width: _size,
      height: _size,
      colorFilter: ColorFilter.mode(
          _color ?? Theme.of(context).colorScheme.onSurface, BlendMode.srcIn),
    );
  }
}
