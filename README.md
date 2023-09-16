# Rust transpiler
###### Attention: this project is still under development, and this readme file, as well as the documentation, are severly lacking

## What is this project?
As the name suggests, this Rust transpiler is a transpiler written in Rust, made to convert my own markup language into html

## What is it's purpose?
As a programmer i've always been interested in generating stuff programatically. But HTML is difficult to generate this way simply because it's syntax is made to be seen, and to be intuitive for programmers, not machines. This means that to generate correct HTML programatically one needs to check many, many conditions, and that just to make sure it is written correctly. Closing and opening tags, for example, or checking tags' tree structure is correct, are some of this conditions, and they can get very tedious to check in code.
**What does this transpiler solve then?**
Well, it isn't the transpiler that solves a problem as much as the markup language it transpilers.
This language, coined S_HTML, for Short HTML, as the longer new name does not suggest,
is a much more simple language than HTML, made of only a few rules, to make the generation of HTML much more simple for computers

## What is a short, understandable example?
S_HTML is very simmilar to html in many ways, but it gets rid of all that's unnecesary or 
overcomplicated.
For example, a paragraph in HTML may look like this
```HTML
<p>Hello, world!</p>
```
But in S_HTML is looks like this

```HTML
-p-Hello, world!
```

This might not look like much, but when dealing with much more complicated HTML, for example

```HTML
<div title="Hello">
    <button> Hello, world! </button>
</div>
```

it simplifies to

```HTML
-div .title="Hello" -
^button- Hello, world!
```

This makes programatically generated HTML much easier to code, as it prevents you having to check if each tag has been closed, or opened, correctly, and only needs you to check the nesting is logically correct.

## How is nesting written?
Nesting has a very simple syntax. Suppose, as above, you have written this code:

```HTML
<div title="Hello">
    <button> Hello, world! </button>
</div>
```

Because the div tag is on the outer most layer, you only write

```HTML
-div .title="Hello" -
```

to declare it. Given that the button element is inside the div, instead of writing

```HTML
-div .title="Hello" -
-button- Hello, world!
```

like you would write normally, you write

```HTML
-div .title="Hello" -
^button- Hello, world!
```

The '^' character signals the button element is inside the div. If you now write

```HTML
-div .title="Hello" -
^button- Hello, world!
-p-This is a paragraph
```

the html code would look like this

```HTML
<div title="Hello">
    <button> Hello, world! </button>
    <p>This is a paragraph</p>
</div>
```

If you wanted the 'p' element to render outside and at the same level the div element is being rendered at, the you write

```HTML
-div .title="Hello" -
^button- Hello, world!
vp-This is a paragraph
```

Notice, this first char in 


```HTML
vp-This is a paragraph
```
is just a 'v', as in **v**ase

This just generates the following HTML:

```HTML
<div title="Hello">
    <button> Hello, world! </button>
</div>
<p>This is a paragraph</p>
```

