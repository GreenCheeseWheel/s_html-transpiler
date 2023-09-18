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
S_HTML is very similar to HTML in many ways, but it gets rid of all that's unnecesary or 
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
Nesting has a very simple syntax. Check out the different ways of writing a p element:

```HTML
-p - This is a paragraph...
^p - This is an nested paragraph
vp - This is a paragraph at the same level the first one is
```

You will notice that, apart from the basic text inside each paragraph, only thing that changes is
the symbol at the start of each line. This symbol (That can only be '-', '^' or 'v') tells you at what level the p element is at. 
If you write '-' at the start of the line, it means this element is at the same level as the element above (So it is not nested inside the element you wrote above it)
A '^' at the start of the line means this element is nested inside the element above it.
A 'v' means you are writing this element a level above, so you are unnesting it.

The above example renders in HTML as:

```HTML
<p> 
    This is a paragraph...
    <p> This is a nested paragraph </p>
</p>

<p>
    This is a paragraph at the same level the first one is
</p>
```

This might seem contrived if you intend to write S_HTML manually, but it makes things easier once you start considering writing a .srth file using code. Not only does this simplify writing HTML programatically, it keeps a simple to remember and write syntax.

## Do tag names change?
No, tag names don't change in S_HTML, so writing a button, a div, or ul element is pretty much the same.

```HTML
-button - This is a button element
-div - This is a div element

-ul -
^li - Unordered list item 1
-li - Unordered list item 2
```

This would render the following HTML:

```HTML
<button> This is a button element </button>
<div> This is a div element </div>

<ul>
    <li>Unordered list item 1</li>
    <li>Unordered list item 2</li>
</ul>
```