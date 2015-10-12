using System.IO;
using sb_bbt.Utils;

namespace sb_bbt.Tests.Fetch
{
    public class TestFetch : Test
    {
        public TestFetch(App app)
            : base(app)
        {
        }

        public override void Run()
        {
            WriteStart();
            Fs.DeleteFolder(App.EnvDir);
            Fs.RunProcess("sb", "fetch", Output);
            if (Directory.Exists(App.SlicesDir))
            {
                WriteFinish(true);
                return;
            }
            WriteFinish(false);
        }
    }
}