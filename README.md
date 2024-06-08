# RHP
A simple HTML preprocessor

## Features

### Custom Simple HTML Tags

Custom tag declarations are done as follows.

```html
<define myCustomTag>
    ...
</define>
```

Note : This declaration has to be done at the top level.
Any declaration not done at the top level is considered invalid and will not be taken into consideration

And any invocation of `myCustomTag` will be replaced by the contents specified inside the define block.
Now let's see in more detail what we can do with it.

#### Children

Children can be attached inside a custom tag.
To do so, simply invoke the `<children/>` tag inside a custom element declaration

Imagine you need to format elements in a specific boilerplate-y way.
You could use custom elements to simplify the syntax

```html
<define imagePreview>
    <div>
        COMPLEX FORMATTING HERE ...
        <children/>
    </div>
</define>

<imagePreview><img src="https://youtube.com/dQw4..."/></imagePreview>
```

##### Children selectors

Maybe you'd like to only insert some of the children ? 
Or split them to insert them in multiple places ?
We've got you covered

```html
<define tagname="example">
    <div>HERE ARE PART OF THE CHILDREN
        <children rhp-select="h1.selector"/>
    </div>
    <div>
        <children rhp-select="h2.selector2"/>
    </div>
</define>

<example>
    <h1 class="selector">I AM SELECTED IN THE FIRST PARTITION</h1>
    <h2 class="selector2">I AM SELECTED IN THE SECOND PARTITION</h2>
</example>
```

The selector used here are standard html selectors.
They are composed of `tagnames`, `.classes`, `#ids`.
Runtime pseudo-classes, such as `::hover()` or `::active()` are not, and will never be accepted. 
(This is a PREprocessor, remember ?)

Standars pseudo classes, such as ::last(), ::nth(n) or ::not(), are not supported yet.

#### Classes and attributes

Any attributes given to your custom tag will be propagated inside of it.
This works for classes, ids, etc.

```html
<myCustomTag class="cool"/>
```

becomes

```html
<div class="cool">...</div>
```

You can also pass string arguments to your custom tags

```html
<define tagname=myCustomTag $src $alt>
    <img src=$src alt=$alt/>
</define>

<myCustomTag src="image_source" alt="image_alt"/>
```

becomes

```html
<img src="image_source" alt="image_alt"/>
```

