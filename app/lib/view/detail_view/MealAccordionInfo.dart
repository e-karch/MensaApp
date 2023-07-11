import 'package:app/view_model/repository/data_classes/meal/Additive.dart';
import 'package:app/view_model/repository/data_classes/meal/Allergen.dart';
import 'package:flutter/cupertino.dart';

class MealAccordionInfo extends StatelessWidget {
  final List<Allergen> _allergens;
  final List<Additive> _additives;

  const MealAccordionInfo(
      {super.key,
      required List<Allergen> allergens,
      required List<Additive> additives})
      : _allergens = allergens,
        _additives = additives;

  @override
  Widget build(BuildContext context) {
    return Column(
      crossAxisAlignment: CrossAxisAlignment.start,
      children: [
        SizedBox(height: 8),
        const Text(
          "Allergene:",
        ),
        ..._allergens.map((e) => Row(
              children: [
                const Text("• "),
                Expanded(child: Text(e.name)),
              ],
            )),
        SizedBox(height: 8),
        const Text(
          "Zusatzstoffe:",
        ),
        ..._additives.map((e) => Row(
              children: [
                const Text("• "),
                Expanded(child: Text(e.name)),
              ],
            )),
      ],
    );
  }
}
