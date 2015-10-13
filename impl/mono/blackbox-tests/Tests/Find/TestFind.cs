using System.IO;
using sb_bbt.Utils;

namespace sb_bbt.Tests.Find
{
    public class TestFind : Test
    {
        public TestFind(App app)
            : base(app)
        {
        }

        public override void Run()
        {
            WriteStart();
            Fs.RunProcess("sb", "find", Output);
            if (Text.ListContains(Output, "jekyll"))
            {
                WriteFinish(true);
                return;
            }
            WriteFinish(false);
        }
    }
}