#pragma once
#include <memory>
#include <stdexcept>
#include <utility>

namespace RDKit {
template<typename T, typename U> std::unique_ptr<T> checked_owned(U &&value) {
	std::unique_ptr<T> result(std::forward<U>(value));
	if (!result) { throw std::invalid_argument("RDKit returned no object"); }
	return result;
}
}
