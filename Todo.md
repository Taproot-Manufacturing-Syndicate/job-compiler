I think svgs can have JavaScript in them that modify the DOM and maybe
that would be a good format for a build artifact from this job compiler

This notion of collapsing the entire job into a single box with all
the inputs and tools and stuff combined and then looking inside the box
and seeing the parallelized build path lines up nicely. With my earlier
thinking of having two separate build artifacts it's really two views
into the same build artifact

What we're currently calling a printed part in the recipes is really
not. It's an arbitrary geometry part that can be produced by printing,
but it can also be produced by whittling or CNC machining or whatever

for making graphs showing the build process:

    plantuml

    graphviz

    dia

    raw svg/html made by the rust program?

The modular recipe namespace should be federated, so that e.g. "light"
in "minimal_hydrponics_setup" can mean something different than "light"
in "headlamp".

Tools should be Things

Process Actions should be full markdown with pictures and stuff


Might want to separate the notion of an Item and a Recipe...  When we're
thinking about how to buy Vitamins we want to think about a box of screws.
When we're thinking about Inputs to a Recipe we want to think about the
individual screws needed.  This might make it easier to support multiple
Recipes for the same Thing?  For now an Item is just a String with the
Item's name.

- Items should be able to have Comments (md with pictures and stuff)

- Recipes say how to make Items


unit coersion in quantities


add some dedicated test repos, don't test with the "real" repos we're
gonna split out later.


each repo should have a `taproot.toml` or `repo.toml` or something that
will point to the external repos that this repo uses, like `Cargo.toml`.


puml doesn't like ':', but accepts '__'


Order of operations is convenient.  Probably the instructions should
produce inputs in the order they're listed in the recipe, rather than
alphabetical...


A recipe should allow a "non-consuming input", for example where something
else that we're making as part of this recipe is used as a guide for
making the current part. This enforces ordering of the steps.


# Vitamins

Vitamins might have a `manufacturer`, distinct from its `vendors`.

Vitamins might cost different amounts from different vendors. :-/

Vitamins should have STLs, eg download fasteners from mcmaster.  Buy them
for now, print them later when we can.

Need a way to annotate Vitamins with words & pics.  This probably relates
to the improved Item abstraction i've been thinking about.

Some way to deal with vitamins that can
be bought separately or in a kit, e.g.
<https://hydrobuilder.com/products/botanicare-ebb-flow-fitting-kit-with-2-extensions>


# mdbook

Include version info in mdbook:

```
  Overview
  We're making minimal_hydroponics_setup.
+ Version: 0.1-10-g123abc (dirty)
```
