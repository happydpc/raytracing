(define-library (raytrace transformations)
  (export scaling translation)
  (import (scheme base)
          (raytrace matrix))
  (begin
    (define (translation dx dy dz)
      (matrix (1 0 0 dx)
              (0 1 0 dy)
              (0 0 1 dz)
              (0 0 0 1)))

    (define (scaling sx sy sz)
      (matrix (sx  0  0 0)
              ( 0 sy  0 0)
              ( 0  0 sz 0)
              ( 0  0  0 1)))))
