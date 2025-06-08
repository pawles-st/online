colors = c("red", "blue", "green", "purple", "orange", "cyan")

requests = 65536

for (d in c(16, 32, 64, 128, 256)) {
	file.pattern <- paste0("result_", d, "_.*", "\\.txt")
	files <- list.files(path = "results", pattern = file.pattern, full.names = TRUE)

	costs.list <- list()
	pages.list <- list()
	for (i in seq_along(files)) {
		file <- files[i]
		y <- read.table(file, header = FALSE, sep = ';')
		costs.list[[i]] <- y[1:requests, 1] / 1:requests
		pages.list[[i]] <- y[1:requests, 2]
	}

	png(paste0("graph_cost_", d, ".png"))
	plot(1, type = "n", xlim = c(1, requests), ylim = range(unlist(costs.list)), xlab = "no. requests", ylab = "avg cost", main = paste0("avg cost ", " (D = ", d, ")"))

	for (i in seq_along(y.list)) {
		y <- costs.list[[i]]
		lines(1:requests, y, col = colors[i])
	}
	legend(x = "topright", legend = c("p = 0.01", "p = 0.02", "p = 0.05", "p = 0.1", "p = 0.2", "p = 0.5"), col = colors, lwd = 3)
	dev.off()

	png(paste0("graph_pages_", d, ".png"))
	plot(1, type = "n", xlim = c(1, requests), ylim = range(unlist(pages.list)), xlab = "no. requests", ylab = "no. pages", main = paste0("number of pages ", " (D = ", d, ")"))

	for (i in seq_along(y.list)) {
		y <- pages.list[[i]]
		lines(1:requests, y, col = colors[i])
	}
	legend(x = "topright", legend = c("p = 0.01", "p = 0.02", "p = 0.05", "p = 0.1", "p = 0.2", "p = 0.5"), col = colors, lwd = 3)
	dev.off()
}



