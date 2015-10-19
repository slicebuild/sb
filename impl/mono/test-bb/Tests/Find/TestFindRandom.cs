using sb.Core.Utils;
using sb.TestBB.Utils;

namespace sb.TestBB.Tests.Find
{
    public class TestFindRandom : Test
    {
        public TestFindRandom(App app)
            : base(app)
        {
        }

        public override void Run()
        {
            WriteStart();
            //todo: pick up a latest slice and use it as the param to find
            Fs.RunProcess("sb", "find ruby", Output);
            if (TextUtils.ListContains(Output, "ruby"))
            {
                WriteFinish(true);
                return;
            }
            WriteFinish(false);
        }
    }
}