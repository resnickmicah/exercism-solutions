(ns scrabble-score (:require [clojure.string :as str]))

(def letter-scores {
   #{\A \E \I \O \U \L \N \R \S \T} 1, 
   #{\D \G} 2, 
   #{\B \C \M \P} 3, 
   #{\F \H \V \W \Y} 4, 
   #{\K} 5, 
   #{\J \X} 8, 
   #{\Q \Z} 10})

(def ls-map (into {} (for [[ks v] letter-scores k ks] [k v])))

(defn score-word
  "Calculate a word's scrabble score"
  [word]
  (apply + (map ls-map (str/upper-case word))))
