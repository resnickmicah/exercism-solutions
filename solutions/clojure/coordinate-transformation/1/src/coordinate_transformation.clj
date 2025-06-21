(ns coordinate-transformation)

(defn translate2d 
  "Returns a function making use of a closure to
   perform a repeatable 2d translation of a coordinate pair."
  [dx dy]
  (let [dx (atom dx) dy (atom dy)]
    (fn [x y]
      [(swap! dx + x) (swap! dy + y)]))
  )

(defn scale2d 
  "Returns a function making use of a closure to
   perform a repeatable 2d scale of a coordinate pair."
  [sx sy]
    (let [sx (atom sx) sy (atom sy)]
    (fn [x y]
      [(swap! sx * x) (swap! sy * y)]))
  )

(defn compose-transform
  "Create a composition function that returns a function that 
   combines two functions to perform a repeatable transformation."
  [f g]
    (fn [x y] (apply g (f x y)))
  )

(defn memoize-transform
  "Returns a function that memoizes the last result.
   If the arguments are the same as the last call,
   the memoized result is returned."
  [f]
  (let [x (atom nil) y (atom nil) result (atom nil)]
    (fn [new-x new-y] 
      (if (and (= new-x @x) (= new-y @y)) 
        @result
        (let [new-result (f new-x new-y)]
          (reset! x new-x) 
          (reset! y new-y)
          (reset! result new-result) 
          new-result)
        )
      )
    )
  )