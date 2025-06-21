(ns elyses-destructured-enchantments)

(defn first-card
  "Returns the first card from deck."
  [deck]
  (let [[fc] deck]
    fc)
)

(defn second-card
  "Returns the second card from deck."
  [deck]
  (let [[fc sc] deck]
    sc)
)

(defn swap-top-two-cards
  "Returns the deck with first two items reversed."
  [deck]
  (let [[fc sc & oc] deck] 
    (if (nil? oc) 
      [sc fc] 
      (vec (concat [sc fc] (vec oc)))))
)

(defn discard-top-card
  "Returns a sequence containing the first card and
   a sequence of the remaining cards in the deck."
  [deck]
  (let [[fc & oc] deck] 
    (if (nil? oc)
      [fc nil]
      [fc (vec oc)]))
)

(def face-cards
  ["jack" "queen" "king"])

(defn insert-face-cards
  "Returns the deck with face cards between its head and tail."
  [deck]
  (let [[fc & oc] deck]
    (if (nil? fc)
      face-cards
      (concat [fc] face-cards (vec oc))
      ))
)
