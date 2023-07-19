import 'package:flutter/material.dart';
import 'IAllergenIcon.dart';

/// This widget is used to display the icon for Rye
class AllergenRyeIcon extends IAllergenIcon {
  const AllergenRyeIcon(
      {super.key, super.width, super.height, super.color});

  @override
  Widget build(BuildContext context) {
    return Text('RO',
        style: TextStyle(
            height: 1.5,
            fontSize: height / 1.5,
            fontWeight: FontWeight.bold,
            color: color));
  }
}
