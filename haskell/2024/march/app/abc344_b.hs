main :: IO ()
main = do
  vn <- getInput []
  mapM_ print $ reverse vn

getInput :: [Int] -> IO [Int]
getInput vn = do
  n <- readLn :: IO Int
  unless
