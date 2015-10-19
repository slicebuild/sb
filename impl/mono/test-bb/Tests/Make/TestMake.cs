using System.IO;
using sb_bbt.Utils;

namespace sb_bbt.Tests.Make
{
    public class TestMake : Test
    {
        public TestMake(App app)
            : base(app)
        {
        }

        public override void Run()
        {
            WriteStart();
            Fs.DeleteFolderExcept(App.EnvDir, App.SlicesDir);
            Fs.RunProcess("sb", "make", Output);
            if (Fs.FolderExists(App.EnvDir, "jekyll"))
            {
                WriteFinish(true);
                return;
            }
            WriteFinish(false);
        }
    }
}