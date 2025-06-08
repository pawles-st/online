colors = c("red", "blue", "green", "purple")

requests = 65536

algorithm.types <- c("CoinFlip", "MoveToMin")
graph.types <- c("Hypercube", "Torus3D")

for (datatype in c("Uniform", "Harmonic", "Biharmonic")) {
	for (d in c(2, 16, 128, 2048)) {
		file.pattern <- paste0("result_.*_.*_", datatype, "_", d, "\\.txt")
		files <- list.files(path = "results", pattern = file.pattern, full.names = TRUE)

		y.list <- list()
		for (i in seq_along(files)) {
			file <- files[i]
			y <- scan(file)
			y.list[[i]] <- y[1:requests] / 1:requests
		}

		png(paste0("graph_", datatype, "_", d, ".png"))
		plot(1, type = "n", xlim = c(1, requests), ylim = range(unlist(y.list)), xlab = "no. requests", ylab = "avg cost", main = paste0(datatype, " (D = ", d, ")"))

		for (i in seq_along(y.list)) {
			y <- y.list[[i]]
			lines(1:requests, y, col = colors[i])
		}
		legend(x = "topright", legend = c("CF + Hypercube", "CF + Torus3D", "MtM + Hypercube", "MtM + Torus3D"), col = colors, lwd = 3)
		dev.off()
	}
}


