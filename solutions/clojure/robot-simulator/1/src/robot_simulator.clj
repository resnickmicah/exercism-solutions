(ns robot-simulator)

(def cw {:north :east :east :south :south :west :west :north})
(def ccw {:north :west :west :south :south :east :east :north})
(def pos-delta {
   :north {:x 0 :y 1}
   :east {:x 1 :y 0}
   :south {:x 0 :y -1}
   :west {:x -1 :y 0}
})

(defn next-pos
  [last-state]
  (let [dir (:bearing last-state) 
        last-pos (:coordinates last-state)
        velocity (dir pos-delta)]
    {:x (+ (:x last-pos) (:x velocity))
     :y (+ (:y last-pos) (:y velocity))})
)

(defn robot
  "Creates a robot at the given coordinates, facing the given direction."
  [coordinates direction]
  ;; function body
  {:coordinates coordinates :bearing direction}
  )

(defn follow-instruction 
  [last-state c]
  (case c
    \R (robot (:coordinates last-state) ((:bearing last-state) cw))
    \L (robot (:coordinates last-state) ((:bearing last-state) ccw))
    \A (robot (next-pos last-state) (:bearing last-state))
  )
)

(defn simulate
  "Simulates the robot's movements based on the given instructions
  and updates its state."
  [instructions robot-state]
  ;; function body
  (reduce follow-instruction robot-state instructions)
  )
