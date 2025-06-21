(ns interest-is-interesting)

(defn interest-rate
  "Returns the interest rate based on the specified balance."
  [balance]
  (cond
    (< balance 0) -3.213
    (< balance 1000) 0.5
    (< balance 5000) 1.621
    :else 2.475)
  )

(defn annual-balance-update
  "Returns the annual balance update, taking into account the interest rate."
  [balance]
  (-> balance
    interest-rate
    bigdec
    (/ 100M)
    (* (abs balance))
    (+ balance))
  )

(defn amount-to-donate
  "Returns how much money to donate based on the balance and the tax-free percentage."
  [balance tax-free-percentage]
  (let [multiplier (if (> balance 0) 2 0)]
    (int (Math/floor (* (abs balance) (/ tax-free-percentage 100) multiplier))))
  )