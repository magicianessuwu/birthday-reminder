# Birthday Reminder
Birthday Reminder is a small tool you can put a link to on your desktop and run it to see if there are any birthdays in the future. I made it because I often forget birthdays of people I care about! (This might break if you enter dates that do not exist or are at days like the 31th.)

## Format
There are multiple tags that allow you to customize how your calendar behaves! If you leave out a time parameter, it will be repeated. For example if you remove the year parameter, the day will repeat every year! They need to be seperated (not ended) by a semicolon (`;`) Here is the list (Name, syntax, example, information):
- Year: `Year:{yearAsInt}` `Year:2024`
- Month: `Month:{monthAsInt}` `Month:04`
- Day: `Day:{dayAsInt}` `Day:01`
- Name: `Name:"{nameHere}"` `Name:"April Fools Day! (Troll all my friends :D)`
- Color: `Color:{colorAsString}` `Color:red` - available colors are: `red`, `green`, `blue`, `yellow`, `pink`, `purple` & `orange`
- Show Entry earlier: `showEarly:{daysAsint}` `showEarly:7`
