const int port = 7654;
var builder = WebApplication.CreateBuilder(args)
.UseUrls($"http://localhost:{port}");
var app = builder.Build();
app.MapGet("/", () => "Hello world!");
app.Run();