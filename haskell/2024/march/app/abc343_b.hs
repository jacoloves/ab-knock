import Control.Monad

main :: IO ()
main = do
  n <- readLn :: IO Int
  graph <- replicateM n $ fmap (map read . words) getLine :: IO [[Int]]

  let adjacencyLists = map (findNeighbors graph) [0 .. n - 1]

  mapM_ (putStrLn . unwords . map show) adjacencyLists

findNeighbors :: [[Int]] -> Int -> [Int]
findNeighbors graph i = [j + 1 | (j, val) <- zip [0 ..] (graph !! i), val == 1]
