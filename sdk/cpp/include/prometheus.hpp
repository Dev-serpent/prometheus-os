#pragma once

#include <string>
#include <functional>
#include <memory>

namespace prometheus {

class AI {
public:
    AI();
    ~AI();

    struct Response {
        std::string text;
        float confidence;
        std::vector<std::string> sources;
    };

    Response query(const std::string& text, const std::string& context = "");
    bool execute(const std::string& action);
    bool analyze_screen();

private:
    class Impl;
    std::unique_ptr<Impl> impl_;
};

class Desktop {
public:
    void open_file(const std::string& path);
    void open_url(const std::string& url);
    void send_notification(const std::string& title, const std::string& body);
    void set_clipboard(const std::string& text);
    std::string get_clipboard();
};

class System {
public:
    std::string execute_command(const std::string& command);
    double cpu_usage();
    struct MemoryInfo {
        uint64_t total;
        uint64_t used;
        uint64_t free;
    };
    MemoryInfo memory_info();
};

class SDK {
public:
    SDK();
    AI& ai();
    Desktop& desktop();
    System& system();
    std::string version();

private:
    AI ai_;
    Desktop desktop_;
    System system_;
};

} // namespace prometheus
