colors = c("red", "blue", "green", "purple", "orange")

items = 100

packer.types <- c("BF", "FF", "NF", "RF", "WF")

for (datatype in c("Uniform", "Harmonic", "Biharmonic", "Geometric")) {
	file.pattern <- paste0("result_.*_", datatype, "\\.txt")
	files <- list.files(path = "results", pattern = file.pattern, full.names = TRUE)

	y.list <- list()
	for (i in seq_along(files)) {
		file <- files[i]
		y <- scan(file)
		y.list[[i]] <- y[1:items]
	}

	png(paste0("graph_", datatype, ".png"))
	plot(1, type = "n", xlim = c(1, items), ylim = range(unlist(y.list)), xlab = "no. items", ylab = "competitive ratio", main = datatype)

	for (i in seq_along(y.list)) {
		y <- y.list[[i]]
		lines(1:items, y, col = colors[i])
	}
	legend(x = "topright", legend = packer.types, col = colors, lwd = 3)
	dev.off()
}


