(ns anagram (:require [clojure.string :as str]))

(defn letter-count [letter word]
  (count (filter #(= letter %) word)))

(defn to-letter-counts [word] 
  (let [letter-set (apply hash-set word)
        letter-counts (map #(letter-count % word) letter-set)]
    (zipmap letter-set letter-counts)))

(defn is-anagram [word prospect] 
  (let [lowercase-target (str/lower-case word)
        lowercase-prospect (str/lower-case prospect)
        target-letters (to-letter-counts lowercase-target)
        not-same-word (not= lowercase-target lowercase-prospect)]
    (->> lowercase-prospect
    to-letter-counts
    (= target-letters)
    (and not-same-word))))

(defn anagrams-for [word prospect-list]
  (filter #(is-anagram word %) prospect-list))
