class PageService {
    constructor($rootScope) {
        this.$rootScope = $rootScope;
        this.config = $rootScope.config;
        this.api = $rootScope.api;

        $rootScope.$on(EventType.SliceScript, (event, args) => {
            this.receiveScript(args.data);
        });
    }

    $rootScope: any;
    config: Config;
    api: ApiService;
    
    sliceName: string;
    sliceOs: string;

    makeScript(form) {
        this.api.getSliceScript();
    }

    receiveScript(data) {
        alert(data);
    }
}