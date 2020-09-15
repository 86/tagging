# tagging

## Overview

tagging is a cli tool to support git tagging easily in Rust.
You can add incremented tags based on sematic versioning without hustle.

## Semantic versioning

Semantic versioning is a simple rule set that dictate how version numbers are assigned and incremented. See details from https://semver.org/.

## Usage

Simply run `tagging`
```
last tags:
0.2.0
0.1.1
0.1.0
:

Which position do you increment?
major (M), minor (m), patch (p)
> 
```

Select incrementing position and return
```
Are you sure you want to add the new tag?

new tag: 0.3.0
commit: 062ac788bc5af463484b9ffef499ba155dc78530 (HEAD -> master)
Author: 86 <triaedz@gmail.com>
Date:   Tue Sep 15 08:43:15 2020 +0900

    cargo new

>
```

Confirm and return

```
Successfully created the new tag!

new tag: 0.3.0
commit: 062ac788bc5af463484b9ffef499ba155dc78530 (HEAD -> master, tag: 0.3.0)
commit: 062ac788bc5af463484b9ffef499ba155dc78530 (HEAD -> master)
Author: 86 <triaedz@gmail.com>
Date:   Tue Sep 15 08:43:15 2020 +0900

    cargo new

>
```

### Notes

The manual input prompt will be shown if tags based on semantic versioning does not exist
```
A tag based on sematic versioning does not exist.
Please input a new tag manually.
> 
```

## Configurations

You can configure options by creating a `tagging.toml` file
```rust
# You can register prefixes to use for tags.
# Tagging ask you which prefix do you use before tagging if you registered them.
prefixes = [
    "v",
    "frontend-",
]

# fetch latest tags (`git fetch --tag`) when launching tagging or not
# default is false
fetch_tags = true

# push the new tags (`git push origin NEW_TAG`) after created a new tag or not
# default is false
push_tag = true
```
