enum EventType {
    SliceScript
}

function start(angular: any, config: Config) {
    angular.module('app', [])
        .run($rootScope => {
            $rootScope.config = config;
        })
        .service('apiService', ApiService)
        .run((
            $rootScope,
            apiService: ApiService
        ) => {
            $rootScope.api = apiService;
        })
        .service('pageService', PageService)
        .run((
            $rootScope,
            pageService: PageService
        ) => {
            $rootScope.page = pageService;
        });
}