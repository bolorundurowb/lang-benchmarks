namespace charp_aspnetcore_plaintext;

class Program
{
    public static void Main(string[] args)
    {
        const int port = 7654;
        var builder = WebApplication.CreateBuilder(args);
        builder.Logging.ClearProviders();
        var app = builder.Build();

        app.MapGet("/", () => "Hello world!");
        Console.WriteLine($"Listening on port {port}");
        app.Run($"http://localhost:{port}");
    }
}