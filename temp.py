
class Point:
    x: int
    y: int
    def __init__(self, x, y):
        self.x = x
        self.y = y
    def __repr__(self):
        return f"Point {{ {self.x}, {self.y} }}"
    def __call__(self, *args, **kwds):
        print(self)

point = Point(1, 2)
print(point())
