colors = c("red", "blue", "green", "purple", "orange", "cyan")

requests = 1000

cache.types <- c("FIFO", "FWF", "LRU", "LFU", "RAND", "RMA")

for (datatype in c("Uniform", "Harmonic", "Biharmonic", "Geometric")) {
	for (n in seq(from = 20, to = 100, by = 10)) {
		for (k in seq(from = n / 10, to = n / 5, by = 1)) {
			file.pattern <- paste0("result_.*_", datatype, "_", n, "_", k, "\\.txt")
			files <- list.files(path = "results", pattern = file.pattern, full.names = TRUE)

			y.list <- list()
			for (i in seq_along(files)) {
				file <- files[i]
				y <- scan(file)
				y <- y / seq_along(y)
				y <- y[(10 * k + 1):requests]
				y.list[[i]] <- y
			}

			png(paste0("graph_", datatype, "_", n, "_", k, ".png"))
			plot(1, type = "n", xlim = c(10 * k + 1, requests), ylim = range(unlist(y.list)), xlab = "requests", ylab = "avg cost", main = paste0(datatype, ", n = ", n, ", k = ", k))

			for (i in seq_along(y.list)) {
				y <- y.list[[i]]
				lines(10 * k + 1:length(y), y, col = colors[i])
			}
			legend(x = "topright", legend = cache.types, col = colors, lwd = 3)
			dev.off()
		}
	}
}

