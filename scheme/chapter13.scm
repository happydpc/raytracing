(import (scheme base)
        (scheme write)
        (scheme file)
        (raytrace tuple)
        (raytrace canvas)
        (raytrace shapes)
        (raytrace matrix)
        (raytrace lights)
        (raytrace material)
        (raytrace transformations)
        (raytrace constants)
        (raytrace camera)
        (raytrace world))

(define floor-material (material (color 1 0.9 0.9)
                                 0.1 0.9 0.0 100.0))

(define floor (plane))
(floor 'set-material! floor-material)

(define middle (cylinder))
(middle 'set-transform! (translation -0.5 1 0.5))
(middle 'set-material! (material (color 0.1 1 0.5)
                                0.1 0.7 0.3 200.0))

(define right (cylinder))
(right 'set-transform! (m4* (translation 1.5 0.0 -0.5)
                            (scaling 0.5 1.0 0.5)))
(right 'set-material! (material (color 0.5 1 0.1)
                               0.1 0.7 0.3 200.0))
(right 'set-maximum! 0.1)
(right 'set-closed! #t)

(define left (cylinder))
(left 'set-transform! (m4* (translation -1.5 0.33 -0.75)
                           (scaling 0.33 0.33 0.33)))
(left 'set-material! (material (color 1 0.8 0.1)
                              0.1 0.7 0.3 200.0))
(left 'set-maximum! 1.0)

(define top (cone))
(top 'set-transform! (m4* (translation 0 0.5 -0.75)
                          (scaling 0.2 0.2 0.2)))
(top 'set-material! (make-material (color 0.9 0.8 0.7)
                                   0.0 0.5 1.0 200.0 0.0 0.5 1.5))
(top 'set-minimum! -1.5)
(top 'set-maximum! -1)
(top 'set-closed! #t)

(define world (make-world (list floor
                                middle
                                right
                                left
                                top)
                          (list (point-light (point -10 10 -10)
                                             (color 1 1 1)))))

(define camera (make-camera 320 160 (/ PI 3)))
(camera 'set-transform! (view-transform (point 0 1.5 -5)
                                        (point 0 1 0)
                                        (vec 0 1 0)))

(define image (camera 'render world))

(call-with-output-file "chapter13.ppm"
  (lambda (f)
    (display (canvas->ppm image) f)))
