# nutrition-calculator
Program that pulls nutritional food information from the Nutritionix API. 
## current features

+ search functionality for a particular food
+ adding that food to a favorite list by pressing "enter"
+ scrolling through foods using left and right arrow keys
+ removing items from favorite list by pressing "d"
## future features
+ keep track of total calories, total fat, total carbs, and various other nutritional information for the favorites list
+ export items to text file for personal use
## known bugs
+ deleting and item from the favorite list, and going to the end of the list will result in an "out of index" panic
	+ for some reason this kills the output to the terminal too. Easy fix coming soon
