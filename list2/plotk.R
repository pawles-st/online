colors = c("red", "blue", "green", "purple", "orange", "cyan")

requests = 1000

cache.types <- c("FIFO", "FWF", "LRU", "LFU", "RAND", "RMA")

for (datatype in c("Uniform", "Harmonic", "Biharmonic", "Geometric")) {
	for (n in seq(from = 20, to = 100, by = 10)) {
		len <- n/5 - n/10 + 1
		file.pattern <- paste0("result_.*_", datatype, "_", n, "\\.txt")
		files <- list.files(path = "results", pattern = file.pattern, full.names = TRUE)

		y.list <- list()
		for (i in seq_along(files)) {
			file <- files[i]
			y <- scan(file)
			y <- y / requests
			y.list[[i]] <- y
		}

		png(paste0("graph2_", datatype, "_", n, ".png"))
		plot(1, type = "n", xlim = c(n/10, n/5), ylim = range(unlist(y.list)), xlab = "k", ylab = "avg cost", main = paste0(datatype, ", n = ", n))

		for (i in seq_along(y.list)) {
			y <- y.list[[i]]
			lines(seq(from = n/10, to = n/5, by = 1), y, col = colors[i])
		}
		legend(x = "topright", legend = cache.types, col = colors, lwd = 3)
		dev.off()
	}
}
