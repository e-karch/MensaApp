import 'dart:math';

import 'package:app/view/core/MensaAppBar.dart';
import 'package:app/view/core/buttons/MensaButton.dart';
import 'package:app/view/core/buttons/MensaIconButton.dart';
import 'package:app/view/core/dialogs/MensaFullscreenDialog.dart';
import 'package:app/view/core/icons/image/ImageReportIcon.dart';
import 'package:app/view/core/icons/image/ThumbDownFilledIcon.dart';
import 'package:app/view/core/icons/image/ThumbDownOutlinedIcon.dart';
import 'package:app/view/core/icons/image/ThumbUpFilledIcon.dart';
import 'package:app/view/core/icons/image/ThumbUpOutlinedIcon.dart';
import 'package:app/view/core/icons/navigation/NavigationCloseIcon.dart';
import 'package:app/view/detail_view/UploadImageDialog.dart';
import 'package:app/view/images/ImageReportDialog.dart';
import 'package:app/view_model/logic/image/IImageAccess.dart';
import 'package:app/view_model/logic/meal/IMealAccess.dart';
import 'package:app/view_model/repository/data_classes/meal/ImageData.dart';
import 'package:app/view_model/repository/data_classes/meal/Meal.dart';
import 'package:app/view_model/repository/error_handling/NoMealException.dart';
import 'package:app/view_model/repository/error_handling/Result.dart';
import 'package:flutter/material.dart';
import 'package:flutter/scheduler.dart';
import 'package:flutter_i18n/flutter_i18n.dart';
import 'package:provider/provider.dart';

/// This widget is used to display the images of a meal.
class MealImageDialog extends StatefulWidget {
  final Meal _meal;

  /// Creates a new meal image dialog.
  const MealImageDialog({super.key, required Meal meal}) : _meal = meal;

  @override
  State<MealImageDialog> createState() => _MealImageDialogState();
}

class _MealImageDialogState extends State<MealImageDialog> {
  int currentPage = 0;
  final PageController pageController = PageController();
  bool isClosing = false;

  @override
  void initState() {
    super.initState();

    pageController.addListener(() {
      if (pageController.page?.round() != currentPage) {
        setState(() {
          currentPage = pageController.page!.round();
        });
      }
    });
  }

  @override
  Widget build(BuildContext context) {
    IImageAccess imageAccess = Provider.of<IImageAccess>(context);
    IMealAccess mealAccess = Provider.of<IMealAccess>(context);
    currentPage = min(currentPage, widget._meal.images!.length - 1);
    return FutureBuilder(
        future: mealAccess.getMeal(widget._meal),
        builder: (context, snapshot) {
          if (snapshot.hasData) {
            switch (snapshot.requireData) {
              case Success<Meal, NoMealException> value:
                Meal meal = value.value;
                if (!isClosing &&
                    (meal.images == null || meal.images!.isEmpty)) {
                  isClosing = true;
                  SchedulerBinding.instance.addPostFrameCallback((_) {
                    Navigator.of(context).pop();
                  });
                }
                ImageData currentImage = meal.images![currentPage];
                return MensaFullscreenDialog(
                    appBar: MensaAppBar(
                        appBarHeight: kToolbarHeight,
                        child: Padding(
                          padding: const EdgeInsets.symmetric(
                              horizontal: 8, vertical: 4),
                          child: Row(
                            children: [
                              MensaIconButton(
                                  semanticLabel: FlutterI18n.translate(
                                      context, "semantics.image.close"),
                                  onPressed: () => Navigator.of(context).pop(),
                                  icon: const NavigationCloseIcon()),
                              const Spacer(),
                            ],
                          ),
                        )),
                    content: PageView.builder(
                      itemCount: (meal.images?.length ?? 0),
                      controller: pageController,
                      itemBuilder: (context, index) {
                        if (index >= meal.images!.length) {
                          return Center(
                              child: MensaButton(
                            semanticLabel: FlutterI18n.translate(
                                context, "semantics.image.upload"),
                            onPressed: () {
                              showDialog(
                                context: context,
                                builder: (context) =>
                                    UploadImageDialog(meal: meal),
                              );
                            },
                            text: FlutterI18n.translate(
                                context, "image.newImageButton"),
                          ));
                        }
                        return Center(
                            child: Image.network(
                          meal.images![index].url,
                          fit: BoxFit.contain,
                        ));
                      },
                    ),
                    actions: (meal.images!.isEmpty ||
                            currentPage >= meal.images!.length ||
                            currentPage == -1)
                        ? const SizedBox(
                            height: 64,
                          )
                        : Padding(
                            padding: const EdgeInsets.symmetric(
                                horizontal: 8, vertical: 8),
                            child: Row(
                              children: [
                                Text(meal.images![currentPage].positiveRating
                                    .toString()),
                                MensaIconButton(
                                    semanticLabel: FlutterI18n.translate(
                                        context,
                                        currentImage.individualRating == 1
                                            ? "semantics.image.ratings.removeUpvote"
                                            : "semantics.image.ratings.addUpvote"),
                                    onPressed: () async {
                                      if (currentImage.individualRating == 1) {
                                        await imageAccess
                                            .deleteUpvote(currentImage);
                                      } else {
                                        await imageAccess
                                            .upvoteImage(currentImage);
                                      }
                                    },
                                    icon: currentImage.individualRating == 1
                                        ? const ThumbUpFilledIcon()
                                        : const ThumbUpOutlinedIcon()),
                                MensaIconButton(
                                    semanticLabel: FlutterI18n.translate(
                                        context,
                                        currentImage.individualRating == -1
                                            ? "semantics.image.ratings.removeDownvote"
                                            : "semantics.image.ratings.addDownvote"),
                                    onPressed: () async {
                                      if (meal.images![currentPage]
                                              .individualRating ==
                                          -1) {
                                        await imageAccess.deleteDownvote(
                                            meal.images![currentPage]);
                                      } else {
                                        await imageAccess.downvoteImage(
                                            meal.images![currentPage]);
                                      }
                                    },
                                    icon: meal.images![currentPage]
                                                .individualRating ==
                                            -1
                                        ? const ThumbDownFilledIcon()
                                        : const ThumbDownOutlinedIcon()),
                                Text(meal.images![currentPage].negativeRating
                                    .toString()),
                                const Spacer(),
                                MensaIconButton(
                                    semanticLabel: FlutterI18n.translate(
                                        context, "semantics.image.report"),
                                    onPressed: () {
                                      showDialog(
                                        context: context,
                                        builder: (context) => ImageReportDialog(
                                            meal: meal,
                                            image: meal.images![currentPage]),
                                      );
                                    },
                                    icon: const ImageReportIcon()),
                              ],
                            )));
              case Failure<Meal, NoMealException> value:
                Navigator.of(context).pop();
                return Center(
                    child: Text(FlutterI18n.translate(
                        context, "image.uploadException.noMeal")));
            }
          } else {
            return const Center(child: CircularProgressIndicator());
          }
        });
  }
}
