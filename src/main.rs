struct ArticleId(i64);

struct ArticleName(String);

enum EArticleTag {
    LIFE,
    TECH,
    OTHER,
}

struct Article {
    id: ArticleId,
    name: ArticleName,
    tag: EArticleTag,
}

fn main() {
    let article = Article {
        id: ArticleId(1),
        name: ArticleName("OOO".into()),
        tag: EArticleTag::OTHER,
    };
}
