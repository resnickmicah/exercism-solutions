(ns resistor-color-duo (:require [clojure.string :as str]))

(def color-values
  {
    "black" 0
    "brown" 1
    "red" 2
    "orange" 3
    "yellow" 4
    "green" 5
    "blue" 6
    "violet" 7
    "grey" 8
    "white" 9
  })

(defn resistor-value
  "Returns the resistor value based on the given colors"
  [colors]
  (as-> colors $
    (map color-values $)
    (str/join $)
    (subs $ 0 2)
    (Integer/parseInt $)))