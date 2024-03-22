main :: IO ()
main = do
  s <- getLine
  putStrLn $ if checkFormat s then "Yes" else "No"

checkFormat :: String -> Bool
checkFormat s
  | length s < 3 = False
  | head s /= '<' || last s /= '>' = False
  | otherwise = all (== '=') (init (tail s))