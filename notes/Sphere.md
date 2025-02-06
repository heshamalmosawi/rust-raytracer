## Equation of a Sphere  

$$ x^2 + y^2 + z^2 = R^2 $$  

We can say that if the given point _(x, y, z)_ is inside the sphere, then:  

$$ x^2 + y^2 + z^2 < R^2 $$  

If the given point is outside the sphere, then:  

$$ x^2 + y^2 + z^2 > R^2 $$  

### Centered at C  

If the center of the sphere is denoted as **C**, then:  

$$ (x - C_x)^2 + (y - C_y)^2 + (z - C_z)^2 = R^2 $$  

Simplified:  

$$ (P - C) \cdot (P - C) = r^2 $$  

We can read this as:  
*"Any point P that satisfies this equation is on the sphere."*  

## Ray-Sphere Intersection  

We want to determine if a ray  

$$ P(t) = A + t b $$  

ever hits the sphere. That means we are looking for any _t_ where:  

$$ (P(t) - C) \cdot (P(t) - C) = r^2 $$  

### Expanding  

Expanding to its full form:  

$$ (A + t \cdot b - C) \cdot (A + t \cdot b - C) = r^2 $$  

Expanding further:  

$$ t^2 (b \cdot b) + 2t (b \cdot (A - C)) + (A - C) \cdot (A - C) - r^2 = 0 $$  

### Quadratic Solution  

The vectors and _r_ are all constants, and _t_ is the unknown. This equation is quadratic, so we solve for its roots.  

The discriminant (from the quadratic formula) determines the number of solutions:  

- **Positive** → Two real solutions (ray intersects the sphere at two points).  
- **Zero** → One real solution (ray just touches the sphere at one point).  
- **Negative** → No real solutions (ray misses the sphere).  
