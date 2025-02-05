# Sphere
- x<sup>2</sup> + y<sup>2</sup> + z<sup>2</sup> = R<sup>2</sup>
<br> We can say that if the given point (_x_, _y_, _z_) is inside the sphere, then x<sup>2</sup> + y<sup>2</sup> + z<sup>2</sup> < R<sup>2</sup>, and if the given point is outside the sphere, then - x<sup>2</sup> + y<sup>2</sup> + z<sup>2</sup> > R<sup>2</sup>.
<br> If the center of the sphere is notated as C, then:
- (x - C<sub>x</sub>)<sup>2</sup> + (y - C<sub>y</sub>)<sup>2</sup> + (z - C<sub>z</sub>)<sup>2</sup> = R<sup>2</sup> <br>
Simplified, we can say:
- (P - C) ⋅ (P - C) = r<sup>2</sup> <br>
We can read this as "any point P that satisifies this equation is on the sphere". What we want to know if our ray `P(t) = A + t b` ever hits the sphere. So we are looking for any _t_ where:
- P((t) - C) ⋅ P((t) - C) = r<sup>2</sup> <br>
Expanded to its full form:
- (A + t ⋅ b - C) ⋅ (A+ t ⋅ b - C) = r<sup>2</sup><br>
Expanded further:
- t<sup>2</sup>b⋅b+2tb⋅(A-C)+(A-C)⋅(A-C)-r<sup>2</sup>=0<br>
The vectors and r are all constants, _t_ s the unknown.. and the equation is quadratic so we could solve it for its roots. The "square root" part of the quadratic formula is called a discriminant. The discriminant can either be positive, negative or zero. A positive discriminant means the it has two real solutions (the ray goes through two points of the sphere), zero (one real solution == interacts once with the object only), or negative (has no real solutions).