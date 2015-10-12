using System.IO;
using System.Reflection;

namespace sb_bbt.App
{
    public class Args
    {
        public string RootDir { get; }
        public string EnvDir { get; }
        public string SlicesDir { get; }

        public Args()
        {
            RootDir = Path.GetDirectoryName(Assembly.GetExecutingAssembly().Location);
            RootDir = RootDir ?? "";
            EnvDir = Path.Combine(RootDir, ".sb");
            SlicesDir = Path.Combine(EnvDir, "slices");
        }

        public void Run()
        {
            
        } 
    }
}