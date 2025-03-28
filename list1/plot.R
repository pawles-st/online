colors = rainbow(4)

n = 100000

list.types = c("FC", "MTF", "Simple", "TP")

for (datatype in c("Uniform", "Harmonic", "Biharmonic", "Geometric")) {
	file.pattern <- paste0("result_.*_", datatype, "\\.txt")

	files <- list.files(pattern = file.pattern, full.names = TRUE)
	print(files)

	y.list <- list()
	for (i in seq_along(files)) {
		file <- files[i]
		y <- scan(file)
		y <- y / seq_along(y)
		y.list[[i]] <- y
	}
	
	png(paste0("graph_", datatype, ".png"))
	plot(1, log = "x", type = "n", xlim = c(1, n), ylim = range(unlist(y.list)), xlab = "n", ylab = "operations", main = datatype)

	for (i in seq_along(y.list)) {
		y <- y.list[[i]]
		lines(1:length(y), y, col = colors[i])
	}
	legend(x = "topleft", legend = list.types, col = colors, lwd = 3)
	dev.off()
}
