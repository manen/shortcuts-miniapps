# `apps/date_tracker`

Track the dates and times of anything \
You press a button and the current time gets added to a list \
Trackes dates inside a JSON database (see [`utils/db`](../../utils/db))

## db

at `<cwd>/db/date_tracker/<date_type_name>.json`:

```jsonc
{
  /// optional: the title of the iphone note to also append the current date to
  "corresponding_note_title":"nem vagyok fuggo",
  "list":["list","of","strings(lines)"]
}
```
