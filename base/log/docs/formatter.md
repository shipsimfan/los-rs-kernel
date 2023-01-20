# Formatter
The formatter takes a log event and converts it to a string. The formatter is described by a string with special entries that get replaced with values from the log event. This allows easy customization of how the logs can be presented.

## Special values
The special values all take the form `{{XX}}` where `XX` is two characters. For all values except the log level, capitalization does not matter. `XX` can take is one of the following values:
 - mo - Displays the module name
 - me - Displays the messages
 - LE - Displays the log level in all capital letters
 - Le - Displays the log level with only the first letter capitalized
 - le - Displays the log level with no capital letters (lE will also do this)

Any other values will be displayed as is.