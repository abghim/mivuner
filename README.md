# Mivuner -- A Simple Derivative Engine Project
Mivuner is a derivative engine for elementary functions (that are, polynomial, log & exp, trig, and combinations of these via arithmetic and power). It produces output by recursively descending and applyling derivative rules, then simplifies to the bare minimum.

Still in initial development. Todo:
- parser; receive input from stdin, then parse into `enum Func`
- improve simplification engine; currently does not handle fraction reducing, combining like terms, and grouping constants in multiplication.
- implicit differentiation; add simple linear solver to find dy/dx
- partial derivatives; differentiate wrt given variable
