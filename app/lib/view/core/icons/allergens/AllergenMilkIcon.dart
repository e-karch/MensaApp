import 'package:flutter/material.dart';
import 'package:flutter_svg/flutter_svg.dart';

import 'AllergenIcon.dart';

/// This widget is used to display the icon for Milk
class AllergenMilkIcon extends AllergenIcon {
  const AllergenMilkIcon(
      {super.key, super.width, super.height, super.color});

  @override
  Widget build(BuildContext context) {
    return SvgPicture.asset('assets/icons/allergens/ml.svg',
        width: width, height: height, colorFilter: ColorFilter.mode(color, BlendMode.srcIn),);
  }
}
