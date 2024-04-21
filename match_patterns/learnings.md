1. In the match patterns arem if some variant has values associated with it as declared ine num then value will bind to it, which can be used inside the arm function
2. Matches in Rust are exhaustive
3. To meet exhaustiveness everytime is not possible, so rust has 'other' keyword to exhaust the possible list, secondly if we wudn't use the value of the other class we can declare it with '_' and if we don't have any value to return we can reutnr unit tuple
4. We can use if let to match only 1 variant and ignore other
