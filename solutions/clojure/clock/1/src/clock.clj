(ns clock (:require [clojure.math :as math]))

(defn clock [hours minutes]
  (let [hours (mod (+ hours (math/floor-div minutes 60)) 24) 
        minutes (mod minutes 60)]
    [hours minutes])
)

(defn clock->string [clock] ;; <- arglist goes here
  (let [[hours minutes] clock]
    (format "%02d:%02d" hours minutes))
)

(defn add-time [old-clock time] ;; <- arglist goes here
  (let [[hours minutes] old-clock]
    (clock hours (+ minutes time)))
)
